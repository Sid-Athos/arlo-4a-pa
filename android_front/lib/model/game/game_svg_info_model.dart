import 'package:miku/model/game/svg_content_model.dart';

class GameSvgInfo {

  String width;
  String height;
  List<SvgContent> svgContent;

  GameSvgInfo(this.width, this.height, this.svgContent);

  String createSVG() {

    String r = "<svg width=\"$width\" height=\"$height\">";

    Map<String, String> parsedStyle = getParsedStyle();

    svgContent.forEach((element) {
      r += element.createSVG(parsedStyle);
    });

    r += "</svg>";
    return r;
  }

  Map<String, String> getParsedStyle() {

    Map<String, String> parsed = {};

    svgContent.forEach((element) {
      if (element.data.containsKey("tag") && element.data["tag"] == "style") {
        parsed = element.parseStyle();
      }
    });

    return parsed;
  }

}