
class CreateUserRequest {
  String pseudo;
  String email;
  String password;

  CreateUserRequest({
    required this.pseudo,
    required this.email,
    required this.password,
  });

  Map<String, dynamic> toJson() => {
    "pseudo": pseudo,
    "email": email,
    "password": password,
  };
}
