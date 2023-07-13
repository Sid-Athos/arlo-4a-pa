import 'package:miku/model/mapper/game_response_mapper.dart';
import 'package:miku/model/mapper/user_response_mapper.dart';
import 'package:miku/model/ranking_model.dart';

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