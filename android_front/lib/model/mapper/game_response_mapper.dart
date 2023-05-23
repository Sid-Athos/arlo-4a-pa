
import 'package:miku/model/game_model.dart';

class GameResponseMapper {

  static Game fromJson(Map<String, dynamic> json) {
    return Game(
      id: json['id'],
      name: json['name'],
      nbPlayer: json['nb_player'],
    );
  }
}