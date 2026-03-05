import 'dart:io';
import 'package:fluster_media_center/src/rust/movie_data/movie_data.dart';
import 'package:flutter/material.dart';

class ItemSnapshot extends StatelessWidget {
  final MovieSnapshot media;

  const ItemSnapshot({super.key, required this.media});

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: null,
      onDoubleTap: () {
        print("double click ${media.title}");
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
              ), // couleur de l'ombre
              blurRadius: 4, // flou
              offset: Offset(4, 8), // décalage horizontal et vertical
            ),
          ],
        ),
        clipBehavior: Clip.antiAlias,
        child: Image.file(
          File(media.posterSnapshot),
          fit: BoxFit.contain, // ou BoxFit.cover selon le rendu voulu
          errorBuilder: (context, error, stackTrace) {
            return const Text("Impossible de charger l'image");
          },
        ),
      ),
    );
  }
}
