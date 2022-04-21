# Changelog

0.4.1
------
* Fixed hanging application after trying to play an unavailable track

0.4.0
------
This is a hotfix release due to a problem with the input method, which resulted in a badly formatted output.
* Changed input method: you need to press enter now to have your input commands recognized

0.3.0
------
* Tracks, Playlists and User profiles can now also be played
* New compact log UI
* New optional ncurses UI (disabled by default)
* New keyboard bindings for quitting, pausing and adjusting volume
* New parameters: `--min-length`, `--max-length` and `--ui`
* Improved API endpoint selection
* The volume is now limited to 0 dBFS

0.2.1
------
* The rank line width is now dependent on the terminal width
* Halkara doesn't crash anymore on timeouts when loading a track
* Internal: updated dependencies

0.2.0
------
* New volume argument (unit is dBFS): `--volume -6.0`
* Internal: updated dependencies

0.1.5
------
* Improved error messages for retrieving trending tracks
* Halkara doesn't crash anymore if a track is blacklisted.

0.1.4
------
* Lower CPU usage through more efficient audio player backend

0.1.3
------
* Improved playback sample rate selection

0.1.2
------
* Switched to an 80 character "heading" on the console
* Internal: the selected API endpoint will now be cached for an hour

0.1.1
------
* Added an ugly workaround to not cut off the end
* *Internal: removed some warnings*

0.1.0
------
* *Initial version*
