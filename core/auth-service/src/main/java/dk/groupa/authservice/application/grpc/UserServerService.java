package dk.groupa.authservice.application.grpc;

import dk.groupa.auth.Service.AuthenticationService;
import dk.groupa.authservice.domain.service.UserCachingService;
import dk.groupa.proto.EmailRequest;
import dk.groupa.proto.EmailResponse;
import dk.groupa.proto.UserServiceGrpc.UserServiceImplBase;
import io.grpc.stub.StreamObserver;
import java.util.List;
import java.util.Set;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import net.devh.boot.grpc.server.service.GrpcService;

@Slf4j
@RequiredArgsConstructor
@GrpcService
public class UserServerService extends UserServiceImplBase {

    private final UserCachingService userCachingService;
    private final AuthenticationService authenticationService;

    @Override
    public StreamObserver<EmailRequest> getAllEmailsOfRoles(StreamObserver<EmailResponse> responseObserver) {
        return new StreamObserver<>() {
            @Override
            public void onNext(EmailRequest emailRequest) {
                Set<String> emailsOfRole = UserServerService.this.userCachingService.getEmailsOfRole(emailRequest.getRole());
                if (!emailsOfRole.isEmpty()) {
                    //  Hit from cache
                    for (String email : emailsOfRole) {
                        responseObserver.onNext(EmailResponse.newBuilder().setEmail(email).build());
                    }
                    return;
                }

                // TODO: call auth service. If we get an okay response, save it in redis
                List<String> emails = authenticationService.getEmailsOfRole(emailRequest.getRole());
                if (emails.isEmpty()) {
                    responseObserver.onError(new Exception("No emails found with role."));
                } else {
                    for (String email : emails) {
                        responseObserver.onNext(EmailResponse.newBuilder().setEmail(email).build());
                    }
                    userCachingService.saveEmailsInRole(emailRequest.getRole(), emails);
                }
            }

            @Override
            public void onError(Throwable throwable) {
                log.error("Error when streaming email responses");
            }

            @Override
            public void onCompleted() {
                responseObserver.onCompleted();
                log.info("Completed email stream");
            }
        };
    }
}
