import 'package:miku/mapper/game/game_response_mapper.dart';
import 'package:miku/mapper/user/user_response_mapper.dart';
import 'package:miku/model/ranking/ranking_model.dart';

class RankingResponseMapper {

  static Ranking fromJson(Map<String, dynamic> json) {
    return Ranking(
      id: json['id'],
      rank: json["rank"],
      user: UserResponseMapper.fromJson(json["user"]),
      game: GameResponseMapper.fromJson(json["game"]),
      nbGames: json["nb_games"],
    );
  }
}