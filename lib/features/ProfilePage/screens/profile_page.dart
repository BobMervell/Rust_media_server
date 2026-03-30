import 'package:fluster_media_center/features/ProfilePage/widgets/profile_summary.dart';
import 'package:fluster_media_center/src/rust/api/media.dart';
import 'package:fluster_media_center/src/rust/domain/person/person_data.dart';
import 'package:flutter/material.dart';

class ProfilePage extends StatelessWidget {
  final int profileId;
  final Color textColor;
  final Color backgroundColor;

  const ProfilePage({
    super.key,
    required this.profileId,
    required this.textColor,
    required this.backgroundColor,
  });

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<PersonData>(
      future: getPerson(personTmdbId: profileId),
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return const Scaffold(
            body: Center(child: CircularProgressIndicator()),
          );
        }

        if (snapshot.hasError) {
          return Scaffold(
            body: Center(child: Text('Erreur: ${snapshot.error}')),
          );
        }

        if (!snapshot.hasData) {
          return const Scaffold(
            body: Center(child: Text("Profil introuvable")),
          );
        }

        final profile = snapshot.data!;

        return Scaffold(
          body: Stack(
            children: [
              CustomScrollView(
                slivers: [
                  SliverToBoxAdapter(
                    child: ProfileSummary(
                      profile: profile,
                      textColor: textColor,
                      backgroundColor: backgroundColor,
                    ),
                  ),
                  const SliverToBoxAdapter(child: SizedBox(height: 40)),
                ],
              ),

              Positioned(
                top: MediaQuery.of(context).padding.top + 12,
                left: 12,
                child: FloatingActionButton(
                  mini: true,
                  backgroundColor: Colors.white,
                  elevation: 0,
                  onPressed: () => Navigator.of(context).pop(),
                  child: const Icon(Icons.arrow_back, color: Colors.black),
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}
