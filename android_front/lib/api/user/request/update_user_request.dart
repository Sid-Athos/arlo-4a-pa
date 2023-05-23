
class UpdateUserRequest {
  String pseudo;

  UpdateUserRequest({
    required this.pseudo,
  });

  Map<String, dynamic> toJson() => {
    "pseudo": pseudo,
  };
}
