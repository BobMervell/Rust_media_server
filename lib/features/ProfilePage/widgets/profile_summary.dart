import 'dart:io';
import 'package:fluster_media_center/src/rust/domain/person/person_data.dart';
import 'package:flutter/material.dart';

class ProfileSummary extends StatelessWidget {
  final PersonData profile;
  final Color textColor;
  final Color backgroundColor;

  const ProfileSummary({
    super.key,
    required this.profile,
    required this.textColor,
    required this.backgroundColor,
  });

  @override
  Widget build(BuildContext context) {
    final screenHeight = MediaQuery.of(context).size.height;

    return Padding(
      padding: EdgeInsetsGeometry.all(20),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Picture(profile: profile, height: screenHeight / 2),
              const SizedBox(width: 16),
              Expanded(
                child: Title(profile: profile, textColor: textColor),
              ),
            ],
          ),
          const SizedBox(height: 20),

          Biography(profile: profile, textColor: textColor),
        ],
      ),
    );
  }
}

class Picture extends StatelessWidget {
  const Picture({super.key, required this.profile, required this.height});

  final PersonData profile;
  final double height;

  @override
  Widget build(BuildContext context) {
    return Container(
      height: height,
      decoration: BoxDecoration(
        borderRadius: const BorderRadius.all(Radius.circular(12)),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withValues(
              alpha: .5,
              red: 0,
              blue: 0,
              green: 0,
            ),
            blurRadius: 4,
            offset: const Offset(4, 8),
          ),
        ],
      ),
      clipBehavior: Clip.antiAlias,
      child: Image.file(
        File(profile.pictureFilePath),
        fit: BoxFit.cover,
        errorBuilder: (context, error, stackTrace) {
          return const Center(child: Text("Impossible de charger l'image"));
        },
      ),
    );
  }
}

class Biography extends StatelessWidget {
  final PersonData profile;
  final Color textColor;

  const Biography({super.key, required this.profile, required this.textColor});

  @override
  Widget build(BuildContext context) {
    return Text(
      profile.biography,
      textAlign: TextAlign.justify,
      style: TextStyle(fontSize: 16, color: textColor),
    );
  }
}

class Title extends StatelessWidget {
  final PersonData profile;
  final Color textColor;

  const Title({super.key, required this.profile, required this.textColor});

  @override
  Widget build(BuildContext context) {
    return Text(
      profile.name,
      textAlign: TextAlign.left,
      style: TextStyle(
        color: textColor,
        fontSize: 42,
        fontWeight: FontWeight.w700,
        letterSpacing: 1.2,
      ),
    );
  }
}
