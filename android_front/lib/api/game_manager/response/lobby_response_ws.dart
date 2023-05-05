import 'dart:developer' as developer;

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/model/lobby_model.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../../../model/lobby_member_model.dart';
import '../../../model/mapper/lobby_member_response_mapper.dart';
import '../../../view/lobby_view.dart';

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

  Lobby toDomainLobby() {
    return Lobby(
        id: id,
        code: code,
        gameId: gameId,
        private: private,
        members: members
    );
  }

  static void compute(Map<String, dynamic> json, BuildContext context, WebSocketChannel channel) {

    LobbyResponseWS messageResponse = LobbyResponseWS.fromJson(json['Lobby']);

    Navigator.push(
        context,
        MaterialPageRoute(builder: (context) => LobbyView(lobby: messageResponse.toDomainLobby(), channel: channel))
    );
  }
}