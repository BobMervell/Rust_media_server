import 'package:fluster_media_center/features/MediaPage/widgets/profile_snapshot.dart';
import 'package:fluster_media_center/src/rust/api/media.dart';
import 'package:fluster_media_center/src/rust/movie_data/movie_data.dart';
import 'package:flutter/material.dart';

class CastScroller extends StatelessWidget {
  final int mediaId;
  final Color textColor;
  final Color backgroundColor;

  const CastScroller({
    super.key,
    required this.mediaId,
    required this.textColor,
    required this.backgroundColor,
  });

  @override
  Widget build(BuildContext context) {
    double height = MediaQuery.of(context).size.height / 2;

    return FutureBuilder<List<PersonSnapshot>>(
      future: getMediaCast(mediaId: mediaId),
      builder: (context, snapshot) {
        if (!snapshot.hasData) {
          return SizedBox(
            height: height,
            child: Center(child: CircularProgressIndicator()),
          );
        }

        final cast = snapshot.data ?? [];

        return Container(
          padding: EdgeInsets.fromLTRB(20, 0, 0, 0),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.end,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                "CAST",
                textAlign: TextAlign.justify,
                style: TextStyle(
                  color: textColor,
                  fontSize: 42,
                  fontWeight: FontWeight.w700,
                  letterSpacing: 1.2,
                ),
              ),
              Divider(
                color: textColor,
                thickness: 1,
                height: 40,
                endIndent: 20,
              ),
              CastScrollView(
                cast: cast,
                height: height,
                textColor: textColor,
                backgroundColor: backgroundColor,
              ),
            ],
          ),
        );
      },
    );
  }
}

class CastScrollView extends StatelessWidget {
  const CastScrollView({
    super.key,
    required this.cast,
    required this.height,
    required this.textColor,
    required this.backgroundColor,
  });

  final List<PersonSnapshot> cast;
  final double height;
  final Color textColor;
  final Color backgroundColor;

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      scrollDirection: Axis.horizontal,
      child: Row(
        children: cast.map((person) {
          return Padding(
            padding: const EdgeInsets.only(right: 24.0),
            child: ProfileSnapshot(
              person: person,
              height: height,
              textColor: textColor,
              backgroundColor: backgroundColor,
            ),
          );
        }).toList(),
      ),
    );
  }
}
