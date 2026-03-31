import 'dart:io';
import 'package:fluster_media_center/src/rust/domain/movie/legacy_moviedata.dart';
import 'package:flutter/material.dart';

class Backdrop extends StatelessWidget {
  final MediaData media;
  const Backdrop({super.key, required this.media});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      child: ShaderMask(
        blendMode: BlendMode.dstIn,
        shaderCallback: (Rect bounds) {
          return bottomFade().createShader(bounds);
        },
        child: ShaderMask(
          blendMode: BlendMode.dstIn,
          shaderCallback: (Rect bounds) {
            return leftFade().createShader(bounds);
          },
          child: Image.file(
            File(media.backdrop),
            fit: BoxFit.contain,
            errorBuilder: (context, error, stackTrace) {
              return const Text("Failed to load backdrop");
            },
          ),
        ),
      ),
    );
  }

  RadialGradient leftFade() {
    return const RadialGradient(
      center: Alignment(-1.9, -0.1),
      radius: 1.4,
      colors: [Colors.transparent, Colors.black],
      stops: [0.7, 1],
    );
  }

  LinearGradient bottomFade() {
    return const LinearGradient(
      begin: Alignment.topCenter,
      end: Alignment.bottomCenter,
      colors: [Colors.black, Colors.transparent],
      stops: [0.70, .9],
    );
  }
}
