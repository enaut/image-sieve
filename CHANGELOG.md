# Changelog
All notable changes to this project will be documented in this file.

## Unreleased

### Changed

 - Changed trash icon

### Added

 - Similars icon in similar images list

### Fixed

 - Removed animated gif from README.md due to size constraints of cargo.io

## [0.5.0] - 2022-02-13

### Changed

 - Renamed all sixtyfps instances to slint
 - Reworked UI and replaced next, open and previous button with on-image buttons, added animations

### Added

 - Support for many raw formats
 - Up and down keys navigate in similar images
 - Enter key opens item

### Fixed

 - Key commands are now working properly after switching tabs, also they have no effect on other tabs than the sort tab
 - Several crashes or weird behavior when images list is empty
 - Discarded status is now properly updated in image caption

## [0.4.2] - 2022-02-05

### Changed

 - Resize video preview image internally to save memory
 - Optimized image loading time

### Added

 - Snap support for removable media and network if permissions are set

### Fixed

 - Do not pass default folder to dir picker if it does not exist
 - Fixed crash when selecting folder without images

## [0.4.2] - 2022-01-09

### Changed

 - Now reading EXIF files for all image formats, not only for JPEGs
 - Optimized loading of similar images
 - Events now have an update button to be able to move an event after or before another event

### Added

 - Filters and sort options to the item list
 - Some symbols in the GUI

### Fixed

 - 

## [0.4.1] - 2022-01-05

### Changed

 - Target directory input field is now always disabled

### Added

 - 

### Fixed

 - Fixed layout in events view

## [0.4.0] - 2022-01-03

### Changed

 - Layout of commit result list improved

### Added

 - Preview for video files
 - Get taken date from video files metadata
 - Showing also mov files now

### Fixed

 - Fixed selected image after changing folder

## [0.3.3] - 2021-12-29

### Changed

 - C runtime is now linked statically for Windows

### Added

 - Setting to select the target directory name pattern

### Fixed

 - Only able to commit when the target directory is set

## [0.3.2] - 2021-12-23

### Changed

 - Performance and robustness improvements
 - Improved layout of events tab

### Added

 - About information in settings tab
 - Help tab

### Fixed

 - Added a space between file info and event name

## [0.3.1] - 2021-12-12

### Changed

 - No directory is now the default
 - Performance checking for files significantly improved

### Added

 - Checking for images can now be cancelled
 - Events are now sorted by date
 - Error message is shown when an event was edited

### Fixed

 - When selecting a folder without images, the similar images model is now cleared
 - Fixed a crash when a folder was selected with insufficient rights to access

## [0.3.0] - 2021-12-06

### Changed

 - Settings are now stored in the home directory. As a consequence, settings from previous versions are lost

### Added

 - Sieving operations are now displayed in detail
 - Generate a MSI installer package for Windows
 - Generate a snap package for Linux

### Fixed

 - Console window is hidden in Windows version


## [0.2.4] - 2021-11-27

### Changed

 - 

### Added

 - Events are now checked for overlapping dates

### Fixed

 - Start date of an event must now be before or equal to the end date

## [0.2.3] - 2021-11-21

### Added

 - Showing result of commit operation now

### Fixed

 - Moving files from one mount point to the other was always failing


## [0.2.2] - 2021-11-14

### Changed

 - All images are now loaded in a background threads increasing GUI responsiveness
 - Improved similarity detection by using longer hashes and taking image orientation into account

### Added

 - Application icon

### Fixed

 - File item date is now the minimum of created and modified date and not only created date
 - Display file item date in local timezone
 - No longer crash when an image with either width or height 0 is loaded
 - Images were cropped in the similar images list
 - Similarities where not calculated when an image was not decodeable


## [0.2.1] - 2021-10-31

### Changed

- Folder selection edit is now disabled, since entering something there had no effect

### Fixed

 - While the image similarity calculation is running, no other folder can be selected


## [0.2.0] - 2021-10-25

### Changed

 - Now using sixtyfps v0.1.4

### Added

 - Image hashing to calculate similarities in image contents
 - Settings tab for tuning the similarity calculation

### Fixed

 - If an image has many similar images, a maximum of six are displayed at the same time since the GUI was blocked otherwise
 - Fixed showing the correct text when one of the similar images was selected
 - Fixed event scrollview


## [0.1.3] - 2021-10-21

### Added

 - Renamed executable to image_sieve instead of image-sieve

### Fixed

 - Fixed crashes that could occur when an item was deleted or renamed while ImageSieve is open 


## [0.1.2] - 2021-10-10

### Added

 - Added a button to open the current item in an external viewer
 - Release to crates.io

### Fixed

 - Improved overall code style
 - Combined code into single crate


## [0.1.1] - 2021-10-10

### Added

 - Added a confirmation when sieving with deletion

### Fixed

 - Fixed GitHub action for releasing Windows binary
 - Fixed updating events


## [0.1.0] - 2021-10-09

### Added

 - Initial GitHub release