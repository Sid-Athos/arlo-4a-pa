import 'package:miku/model/svg_content_model.dart';

class GameSvgInfo {

  String width;
  String height;
  List<SvgContent> svgContent;

  GameSvgInfo(this.width, this.height, this.svgContent);

  String createSVG() {

    String r = "<svg width=\"$width\" height=\"$height\">";

    svgContent.forEach((element) {
      r += element.createSVG();
    });

    r += "</svg>";
    return r;
  }

}