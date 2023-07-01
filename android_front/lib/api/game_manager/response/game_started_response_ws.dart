import 'dart:developer' as developer;

import 'package:miku/model/game_started.dart';

import '../../../model/game_model.dart';
import '../../../model/lobby_member_model.dart';
import '../../../model/mapper/game_response_mapper.dart';
import '../../../model/mapper/lobby_member_response_mapper.dart';

class GameStartedResponseWS {
  final int id;
  final List<LobbyMember> members;
  final Game game;

  GameStartedResponseWS({
    required this.id,
    required this.members,
    required this.game,
  });

  factory GameStartedResponseWS.fromJson(Map<String, dynamic> json) {
    return GameStartedResponseWS(
      id: json['id'],
      members: (json['members'] as List<dynamic>)
          .map((member) => LobbyMemberResponseMapper.fromJson(member))
          .toList(),
      game: GameResponseMapper.fromJson(json['game']),
    );
  }

  GameStarted toDomainLobby() {
    return GameStarted(
      id: id,
      members: members,
      game: game,
    );
  }
}
