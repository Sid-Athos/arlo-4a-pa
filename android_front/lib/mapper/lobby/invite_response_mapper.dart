import 'package:miku/model/lobby/invite_model.dart';
import 'package:miku/mapper/lobby/lobby_response_mapper.dart';
import 'package:miku/mapper/user/user_response_mapper.dart';

class InviteResponseMapper {

  static Invite fromJson(Map<String, dynamic> json) {
    return Invite(
      from: UserResponseMapper.fromJson(json['from_user']),
      to: UserResponseMapper.fromJson(json['to_user']),
      lobby: LobbyResponseMapper.fromJson(json["lobby"])
    );
  }
}