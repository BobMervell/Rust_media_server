import 'package:fluster_media_center/features/MediaPage/widgets/profile_snapshot.dart';
import 'package:fluster_media_center/src/rust/api/media.dart';
import 'package:fluster_media_center/src/rust/movie_data/movie_data.dart';
import 'package:flutter/material.dart';

class CrewScroller extends StatelessWidget {
  final int mediaId;
  final Color textColor;
  const CrewScroller({
    super.key,
    required this.mediaId,
    required this.textColor,
  });

  @override
  Widget build(BuildContext context) {
    double height = MediaQuery.of(context).size.height / 2;

    return FutureBuilder<List<PersonData>>(
      future: getMediaCrew(mediaId: mediaId),
      builder: (context, snapshot) {
        if (!snapshot.hasData) {
          return SliverToBoxAdapter(
            child: SizedBox(
              height: height,
              child: Center(child: CircularProgressIndicator()),
            ),
          );
        }

        final crew = snapshot.data ?? [];

        return SliverToBoxAdapter(
          child: Container(
            padding: EdgeInsets.fromLTRB(20, 0, 0, 0),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.end,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  "CREW",
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
                CrewScrollView(
                  crew: crew,
                  height: height,
                  textColor: textColor,
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}

class CrewScrollView extends StatelessWidget {
  const CrewScrollView({
    super.key,
    required this.crew,
    required this.height,
    required this.textColor,
  });

  final List<PersonData> crew;
  final double height;
  final Color textColor;

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      scrollDirection: Axis.horizontal,
      child: Row(
        children: crew.map((person) {
          return Padding(
            padding: const EdgeInsets.only(right: 24.0),
            child: ProfileSnapshot(
              person: person,
              height: height,
              textColor: textColor,
            ),
          );
        }).toList(),
      ),
    );
  }
}
