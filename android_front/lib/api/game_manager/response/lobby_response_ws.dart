import 'dart:developer' as developer;

import '../../../model/lobby_member_model.dart';
import '../../../model/mapper/lobby_member_response_mapper.dart';

class LobbyResponseWS {
  final int id;
  final String code;
  final int gameId;
  final bool private;
  final List<LobbyMember> members;

  LobbyResponseWS({
    required this.id,
    required this.code,
    required this.gameId,
    required this.private,
    required this.members,
  });

  factory LobbyResponseWS.fromJson(Map<String, dynamic> json) {
    return LobbyResponseWS(
      id: json['id'],
      code: json['code'],
      gameId: json['game_id'],
      private: json['private'],
      members: (json['members'] as List<dynamic>).map((member) => LobbyMemberResponseMapper.fromJson(member)).toList(),
    );
  }

  static void compute(Map<String, dynamic> json) {

    LobbyResponseWS messageResponse = LobbyResponseWS.fromJson(json['Lobby']);

    developer.log(messageResponse.id.toString());
    developer.log(messageResponse.code);
    developer.log(messageResponse.gameId.toString());
    developer.log(messageResponse.private.toString());
    developer.log(messageResponse.members.toString());
  }
}