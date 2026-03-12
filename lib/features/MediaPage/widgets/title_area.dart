import 'package:fluster_media_center/src/rust/api/media.dart';
import 'package:fluster_media_center/src/rust/movie_data/movie_data.dart';
import 'package:flutter/material.dart';

class TitleArea extends StatelessWidget {
  final MediaData media;
  final Color textColor;
  const TitleArea({super.key, required this.media, required this.textColor});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(20.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.center,
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Title(media: media, textColor: textColor),
          if (media.title != media.originalTitle) ...[
            const SizedBox(height: 4),
            OriginalTitle(media: media, textColor: textColor),
          ],
          const SizedBox(height: 8),
          PlayBtn(media: media),
          SizedBox(height: 20),
          Summary(media: media, textColor: textColor),
        ],
      ),
    );
  }
}

class Summary extends StatelessWidget {
  const Summary({super.key, required this.media, required this.textColor});

  final MediaData media;
  final Color textColor;

  @override
  Widget build(BuildContext context) {
    return Text(
      media.summary,
      textAlign: TextAlign.justify,
      style: TextStyle(fontSize: 16, color: textColor),
    );
  }
}

class PlayBtn extends StatelessWidget {
  const PlayBtn({super.key, required this.media});

  final MediaData media;

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: () async {
        String realPath = "/mnt/smb/fluster/${media.filePath}";
        await tempoMountSmb();
        openVideo(path: realPath);
      },
      child: const Text("Play"),
    );
  }
}

class OriginalTitle extends StatelessWidget {
  const OriginalTitle({
    super.key,
    required this.media,
    required this.textColor,
  });

  final MediaData media;
  final Color textColor;

  @override
  Widget build(BuildContext context) {
    return Text(
      media.originalTitle,
      style: TextStyle(
        color: textColor.withAlpha(200),
        fontSize: 20,
        fontStyle: FontStyle.italic,
      ),
    );
  }
}

class Title extends StatelessWidget {
  const Title({super.key, required this.media, required this.textColor});

  final MediaData media;
  final Color textColor;

  @override
  Widget build(BuildContext context) {
    return Text(
      media.title,
      textAlign: TextAlign.center,
      style: TextStyle(
        color: textColor,
        fontSize: 42,
        fontWeight: FontWeight.w700,
        letterSpacing: 1.2,
      ),
    );
  }
}
