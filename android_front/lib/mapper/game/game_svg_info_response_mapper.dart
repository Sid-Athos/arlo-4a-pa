import 'package:miku/model/game/game_svg_info_model.dart';
import 'package:miku/mapper/game/svg_content_response_mapper.dart';

class GameSvgInfoResponseMapper {

  static GameSvgInfo fromJson(Map<String, dynamic> json) {
    return GameSvgInfo(
        json["width"],
        json["height"],
        (json['content'] as List<dynamic>).map((e) => SvgContentResponseMapper.fromJson(e)).toList()
    );
  }
}