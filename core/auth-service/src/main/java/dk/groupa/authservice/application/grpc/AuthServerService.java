package dk.groupa.authservice.application.grpc;

import dk.groupa.proto.AuthRequest;
import dk.groupa.proto.AuthResponse;
import dk.groupa.proto.AuthServiceGrpc.AuthServiceImplBase;
import io.grpc.stub.StreamObserver;
import net.devh.boot.grpc.server.service.GrpcService;

@GrpcService
public class AuthServerService extends AuthServiceImplBase {

    @Override
    public void isUserAuthenticatedWithRole(AuthRequest request, StreamObserver<AuthResponse> responseObserver) {
        responseObserver.onNext(AuthResponse.newBuilder().setIsAuthenticated(true).build());
        responseObserver.onCompleted();
    }
}
