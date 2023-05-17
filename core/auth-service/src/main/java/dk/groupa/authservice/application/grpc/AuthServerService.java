package dk.groupa.authservice.application.grpc;

import dk.groupa.auth.Model.AuthDTO;
import dk.groupa.auth.Service.AuthenticationService;
import dk.groupa.proto.AuthRequest;
import dk.groupa.proto.AuthResponse;
import dk.groupa.proto.AuthResponse.Builder;
import dk.groupa.proto.AuthServiceGrpc.AuthServiceImplBase;
import dk.groupa.proto.User;
import io.grpc.stub.StreamObserver;
import lombok.RequiredArgsConstructor;
import net.devh.boot.grpc.server.service.GrpcService;

@GrpcService
@RequiredArgsConstructor
public class AuthServerService extends AuthServiceImplBase {

    private final AuthenticationService authenticationService;

    @Override
    public void isUserAuthenticatedWithRole(AuthRequest request, StreamObserver<AuthResponse> responseObserver) {
        AuthDTO userAuthenticatedWithRole = this.authenticationService.isUserAuthenticatedWithRole(
            request.getAuthToken(),
            request.getRole()
        );
        Builder builder = AuthResponse.newBuilder()
            .setIsAuthenticated(userAuthenticatedWithRole.isAuthenticated());
        userAuthenticatedWithRole.getUser().ifPresent(user -> {
            builder.setUser(User.newBuilder()
                .setEmail(user.getEmail())
                .setRole(user.getRole())
                .setId(user.getId())
                .build()
            );
        });
        responseObserver.onNext(builder.build());
        responseObserver.onCompleted();
    }
}
