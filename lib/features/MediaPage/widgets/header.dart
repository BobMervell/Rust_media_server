import 'package:fluster_media_center/features/MediaPage/widgets/backdrop.dart';
import 'package:fluster_media_center/features/MediaPage/widgets/title_area.dart';
import 'package:fluster_media_center/src/rust/movie_data/movie_data.dart';
import 'package:flutter/material.dart';

class Header extends StatelessWidget {
  const Header({
    super.key,
    required this.media,
    required this.textColor,
    required this.backgroundColor,
  });

  final MediaData media;
  final Color textColor;
  final Color backgroundColor;

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        Positioned(
          top: 0,
          right: -MediaQuery.of(context).size.width / 10,
          width: MediaQuery.of(context).size.width * .95,
          child: Backdrop(media: media),
        ),

        Positioned(
          top: 20,
          left: 20,
          width: MediaQuery.of(context).size.width * 1 / 3,
          height: MediaQuery.of(context).size.height * 3 / 5,
          child: TitleArea(media: media, textColor: textColor),
        ),

        Positioned(
          top: MediaQuery.of(context).padding.top + 12,
          left: 12,
          child: FloatingActionButton(
            mini: true,
            backgroundColor: textColor,
            elevation: 0,
            onPressed: () {
              Navigator.of(context).pop();
            },
            child: Icon(Icons.arrow_back, color: backgroundColor),
          ),
        ),
      ],
    );
  }
}
