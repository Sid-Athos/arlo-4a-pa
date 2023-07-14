
import 'package:miku/model/svg_content_model.dart';

class SvgContentResponseMapper {

  static SvgContent fromJson(Map<String, dynamic> data) {
    Map<String, String> realData = Map();

    data.forEach((key, value) {
      realData[key] = value.toString();
    });

    return SvgContent(
      realData
    );
  }
}