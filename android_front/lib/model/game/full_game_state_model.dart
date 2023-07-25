import 'package:miku/model/game/game_state_model.dart';

import 'game_action_model.dart';
import 'game_state_model.dart';
import 'game_svg_info_model.dart';

class FullGameState {

  List<GameSvgInfo> displays;
  List<GameAction> requested_actions;
  GameState game_state;

  FullGameState({
    required this.displays,
    required this.requested_actions,
    required this.game_state,
  });

}