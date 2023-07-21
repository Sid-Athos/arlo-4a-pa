
import 'package:miku/model/game/game_model.dart';

class GameResponseMapper {

  static Game fromJson(Map<String, dynamic> json) {
    return Game(
      id: json['id'],
      name: json['name'],
      description: json['description'],
      minPlayers: json['min_players'],
      maxPlayers: json['max_players']
    );
  }
}