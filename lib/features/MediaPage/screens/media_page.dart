import 'dart:io';

import 'package:fluster_media_center/features/MediaPage/widgets/cast_scroller.dart';
import 'package:fluster_media_center/features/MediaPage/widgets/crew_scroller.dart';
import 'package:fluster_media_center/features/MediaPage/widgets/header.dart';
import 'package:fluster_media_center/src/rust/api/media.dart';
import 'package:flutter/material.dart';
import 'package:flutter_color_extractor/flutter_color_extractor.dart';

class MediaPage extends StatelessWidget {
  final int mediaId;

  const MediaPage({super.key, required this.mediaId});

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<List<dynamic>>(
      future: Future.wait([getMedia(mediaId: mediaId)]).then((results) async {
        final media = results[0];
        final color = await getDominantColor(media.backdrop);
        return [media, color];
      }),
      builder: (context, snapshot) {
        if (!snapshot.hasData) {
          return const Scaffold(
            body: Center(child: CircularProgressIndicator()),
          );
        }

        final media = snapshot.data![0];
        final dominantColor =
            snapshot.data![1] ?? const Color.fromARGB(255, 160, 180, 180);

        final textColor = getReadableTextColor(dominantColor);

        return Scaffold(
          backgroundColor: dominantColor,
          body: CustomScrollView(
            key: ValueKey(dominantColor),
            slivers: [
              HeaderCastWrapper(
                media: media,
                textColor: textColor,
                backgroundColor: dominantColor,
                mediaId: mediaId,
              ),
              SliverToBoxAdapter(child: SizedBox(height: 40)),
              CrewScroller(
                mediaId: mediaId,
                textColor: textColor,
                backgroundColor: dominantColor,
              ),
              SliverToBoxAdapter(child: SizedBox(height: 100)),
            ],
          ),
        );
      },
    );
  }
}

class HeaderCastWrapper extends StatelessWidget {
  const HeaderCastWrapper({
    super.key,
    required this.media,
    required this.textColor,
    required this.backgroundColor,
    required this.mediaId,
  });

  final dynamic media;
  final Color textColor;
  final Color backgroundColor;
  final int mediaId;

  @override
  Widget build(BuildContext context) {
    return SliverToBoxAdapter(
      child: SizedBox(
        height: MediaQuery.of(context).size.height * 1.5,
        child: Stack(
          children: [
            Header(
              media: media,
              textColor: textColor,
              backgroundColor: backgroundColor,
            ),
            Align(
              alignment: AlignmentGeometry.bottomRight,
              child: CastScroller(
                mediaId: mediaId,
                textColor: textColor,
                backgroundColor: backgroundColor,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

Future<Color> getDominantColor(String imagePath) async {
  try {
    final extractor = const ColorExtractor();

    final bytes = await File(
      imagePath,
    ).readAsBytes().timeout(const Duration(milliseconds: 300));

    final colors = await extractor
        .extractColorsFromBytes(bytes)
        .timeout(const Duration(milliseconds: 300));

    if (colors.isEmpty) {
      return Colors.black;
    }

    return colors.first;
  } catch (_) {
    return Colors.black;
  }
}

Color getReadableTextColor(Color background) {
  return background.computeLuminance() > 0.5 ? Colors.black : Colors.white;
}
