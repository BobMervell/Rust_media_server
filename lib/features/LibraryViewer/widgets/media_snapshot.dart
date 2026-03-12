import 'dart:io';
import 'package:fluster_media_center/features/MediaPage/screens/media_page.dart';
import 'package:fluster_media_center/src/rust/api/media.dart';
import 'package:fluster_media_center/src/rust/movie_data/movie_data.dart';
import 'package:flutter/material.dart';

class MediaSnapshot extends StatelessWidget {
  final MovieSnapshot media;

  const MediaSnapshot({super.key, required this.media});

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: () async {
        Navigator.push(
          context,
          MaterialPageRoute(builder: (context) => MediaPage(mediaId: media.id)),
        );
      },
      onDoubleTap: () async {
        String realPath = "/mnt/smb/fluster/${media.filePath}";

        await tempoMountSmb();

        //TODO (test) integrated player)

        // Navigator.push(
        //   context,
        //   MaterialPageRoute(builder: (context) => SecondPage(path: realPath)),
        // );

        openVideo(path: realPath);
      },

      child: Container(
        decoration: BoxDecoration(
          borderRadius: BorderRadius.all(Radius.circular(12)),
          boxShadow: [
            BoxShadow(
              color: Colors.black.withValues(
                alpha: .5,
                red: 0,
                blue: 0,
                green: 0,
              ),
              blurRadius: 4,
              offset: Offset(4, 8),
            ),
          ],
        ),
        clipBehavior: Clip.antiAlias,
        child: Image.file(
          File(media.poster),
          fit: BoxFit.contain, // ou BoxFit.cover selon le rendu voulu
          errorBuilder: (context, error, stackTrace) {
            return const Text("Impossible de charger l'image");
          },
        ),
      ),
    );
  }
}
