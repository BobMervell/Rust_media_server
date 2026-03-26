import 'dart:io';
import 'package:fluster_media_center/features/ProfilePage/screens/profile_page.dart';
import 'package:fluster_media_center/src/rust/movie_data/movie_data.dart';
import 'package:flutter/material.dart';

class ProfileSnapshot extends StatelessWidget {
  final PersonSnapshot person;
  final double height;
  final Color textColor;
  final Color backgroundColor;

  const ProfileSnapshot({
    super.key,
    required this.person,
    required this.height,
    required this.textColor,
    required this.backgroundColor,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        GestureDetector(
          onTap: () async {
            Navigator.push(
              context,
              MaterialPageRoute(
                builder: (context) => ProfilePage(
                  profileId: person.tmdbId,
                  textColor: textColor,
                  backgroundColor: backgroundColor,
                ),
              ),
            );
          },
          child: ProfilePicture(height: height, person: person),
        ),
        SizedBox(height: 20),
        ProfileName(person: person, textColor: textColor),
        ProfileRole(person: person, textColor: textColor),
      ],
    );
  }
}

class ProfilePicture extends StatelessWidget {
  const ProfilePicture({super.key, required this.height, required this.person});

  final double height;
  final PersonSnapshot person;

  @override
  Widget build(BuildContext context) {
    return Container(
      height: height,
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
        File(person.picturePath),
        fit: BoxFit.contain,
        errorBuilder: (context, error, stackTrace) {
          return const Text("Impossible de charger l'image");
        },
      ),
    );
  }
}

class ProfileName extends StatelessWidget {
  const ProfileName({super.key, required this.person, required this.textColor});

  final PersonSnapshot person;
  final Color textColor;

  @override
  Widget build(BuildContext context) {
    return Text(
      person.name,
      style: TextStyle(
        fontSize: 20,
        color: textColor,
        fontWeight: FontWeight.w700,
      ),
    );
  }
}

class ProfileRole extends StatelessWidget {
  const ProfileRole({super.key, required this.person, required this.textColor});

  final PersonSnapshot person;
  final Color textColor;

  @override
  Widget build(BuildContext context) {
    return Text(
      person.jobName.toLowerCase() == 'actor'
          ? person.character
          : person.jobName,
      style: TextStyle(
        fontSize: 16,
        color: textColor,
        fontStyle: FontStyle.italic,
      ),
    );
  }
}
