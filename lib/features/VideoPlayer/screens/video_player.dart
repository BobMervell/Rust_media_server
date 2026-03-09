import 'package:flutter/material.dart';
import 'package:media_kit/media_kit.dart'; // Provides [Player], [Media], [Playlist] etc.
import 'package:media_kit_video/media_kit_video.dart'; // Provides [VideoController] & [Video] etc.

class VideoPlayer extends StatefulWidget {
  final String path;
  const VideoPlayer({super.key, required this.path});

  @override
  State<VideoPlayer> createState() => VideoPlayerState();
}

class VideoPlayerState extends State<VideoPlayer> {
  late final player = Player(
    configuration: PlayerConfiguration(
      protocolWhitelist: [
        'udp',
        'rtp',
        'tcp',
        'tls',
        'data',
        'file',
        'http',
        'https',
        'crypto',
        'smb',
      ],
      logLevel: MPVLogLevel.debug, // pour plus de détails
    ),
  );
  late final controller = VideoController(player);

  @override
  void initState() {
    super.initState();
    player.open(Media(widget.path));
  }

  @override
  void dispose() {
    player.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: SizedBox(
        width: MediaQuery.of(context).size.width,
        height: MediaQuery.of(context).size.width * 9.0 / 16.0,
        child: Video(controller: controller),
      ),
    );
  }
}

class SecondPage extends StatelessWidget {
  final String path;
  const SecondPage({super.key, required this.path});
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text("Video page")),
      body: VideoPlayer(path: path),
    );
  }
}
