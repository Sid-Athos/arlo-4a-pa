
class LobbyMember {

  int id;
  String pseudo;
  String email;
  bool admin;
  bool isHost;

  LobbyMember({
    required this.id,
    required this.pseudo,
    required this.email,
    required this.admin,
    required this.isHost,
  });
}
