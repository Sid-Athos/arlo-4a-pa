
import '../../../model/lobby_member_model.dart';

class LobbyMemberResponseMapper {

  static LobbyMember fromJson(Map<String, dynamic> json) {
    return LobbyMember(
      id: json['id'],
      pseudo: json['pseudo'],
      email: json['email'],
      admin: json['admin'],
      isHost: json['isHost'],
    );
  }
}