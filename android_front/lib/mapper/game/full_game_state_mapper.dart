
import 'package:miku/mapper/game/game_action_response_mapper.dart';
import 'package:miku/mapper/game/game_state_response_mapper.dart';
import 'package:miku/mapper/game/game_svg_info_response_mapper.dart';

import '../../model/game/full_game_state_model.dart';

class FullGameStateMapper {

  static FullGameState fromJson(Map<String, dynamic> json) {
    return FullGameState(
        displays: (json['displays'] as List<dynamic>).map((e) => GameSvgInfoResponseMapper.fromJson(e)).toList(),
        requested_actions: (json['requested_actions'] as List<dynamic>).map((e) => GameActionResponseMapper.fromJson(e)).toList(),
        game_state: GameStateMapper.fromJson(json["game_state"])
    );
  }
}