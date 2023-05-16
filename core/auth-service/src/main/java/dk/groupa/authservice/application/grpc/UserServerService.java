package dk.groupa.authservice.application.grpc;

import dk.groupa.authservice.domain.service.UserCachingService;
import dk.groupa.proto.EmailRequest;
import dk.groupa.proto.EmailResponse;
import dk.groupa.proto.UserServiceGrpc.UserServiceImplBase;
import io.grpc.stub.StreamObserver;
import java.util.List;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import net.devh.boot.grpc.server.service.GrpcService;

@Slf4j
@RequiredArgsConstructor
@GrpcService
public class UserServerService extends UserServiceImplBase {

    private final UserCachingService userCachingService;

    @Override
    public StreamObserver<EmailRequest> getAllEmailsOfRoles(StreamObserver<EmailResponse> responseObserver) {
        return new StreamObserver<>() {
            @Override
            public void onNext(EmailRequest emailRequest) {
                List<String> emailsOfRole = UserServerService.this.userCachingService.getEmailsOfRole(emailRequest.getRole());
                if (!emailsOfRole.isEmpty()) {
                    //  Hit from cache
                    for (String email : emailsOfRole) {
                        responseObserver.onNext(EmailResponse.newBuilder().setEmail(email).build());
                    }
                    responseObserver.onCompleted();
                    return;
                }

                // TODO: call auth service here
                responseObserver.onError(new Exception("No emails found with role."));
                responseObserver.onCompleted();
            }

            @Override
            public void onError(Throwable throwable) {
                log.error("Error when streaming email responses");
            }

            @Override
            public void onCompleted() {
                log.info("Completed email stream");
            }
        };
    }
}
