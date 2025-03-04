# Building and Testing for Android TV

This guide will help you build and test the IPTV application for Android TV devices.

## Prerequisites

- Android Studio (latest version recommended)
- Android SDK with Android TV components installed
- A physical Android TV device or emulator
- ADB (Android Debug Bridge) installed

## Setting Up the Development Environment

1. **Install Android Studio**
   - Download from [developer.android.com/studio](https://developer.android.com/studio)
   - During installation, ensure you select the Android TV SDK components

2. **Configure Android TV Emulator** (if not using a physical device)
   - Open Android Studio → Tools → AVD Manager
   - Click "Create Virtual Device"
   - Select "TV" category and choose an Android TV device (e.g., Android TV 1080p)
   - Select a system image (at least Android 9.0 Pie, API Level 28 recommended)
   - Configure remaining settings and finish the setup

## Building for Android TV

1. **Configure the project for Android TV**
   - Ensure your `AndroidManifest.xml` includes TV-specific intents and requirements:
   ```xml
   <uses-feature android:name="android.software.leanback" android:required="false" />
   <uses-feature android:name="android.hardware.touchscreen" android:required="false" />
   ```

2. **Add Leanback support library**
   - In your app's `build.gradle` file, add:
   ```gradle
   implementation 'androidx.leanback:leanback:1.0.0'
   ```

3. **Building the APK**
   - From Android Studio: Build → Build Bundle(s) / APK(s) → Build APK(s)
   - Or use Gradle from the command line:
   ```bash
   ./gradlew assembleDebug
   ```

## Installing on Android TV

### Using ADB (Debug mode)

1. **Enable Developer Options on your Android TV**
   - Go to Settings → Device Preferences → About
   - Click on Build number 7 times to enable developer options

2. **Enable USB debugging**
   - Go to Settings → Device Preferences → Developer options
   - Enable "USB debugging"

3. **Connect to your Android TV**
   - If using a physical device, get its IP address from network settings
   - Connect via ADB:
   ```bash
   adb connect [TV_IP_ADDRESS]
   ```
   - If using an emulator, it should be automatically detected

4. **Install the app**
   ```bash
   adb install -r [path_to_your_apk]
   ```

### Using Google Play Store

1. Create a release version of the app
2. Sign the APK with a release key
3. Follow the Google Play publishing process specific to TV apps

## Testing on Android TV

### Manual Testing

1. **UI Navigation Testing**
   - Ensure all UI elements can be navigated using the D-pad
   - Test the focus behavior of all interactive elements
   - Verify that the selected items are clearly highlighted

2. **Performance Testing**
   - Check for smooth channel switching
   - Monitor memory usage during extended viewing
   - Test how the app handles network fluctuations

3. **Media Playback Testing**
   - Test various stream formats (HLS, DASH, etc.)
   - Verify that playback controls work properly with the remote
   - Test seeking, pausing, and resuming content

### Automated Testing

1. **UI Tests with Espresso**
   - Create tests specific to TV navigation patterns
   ```java
   @RunWith(AndroidJUnit4.class)
   public class TvNavigationTest {
       @Rule
       public ActivityTestRule<MainTvActivity> activityRule = 
           new ActivityTestRule<>(MainTvActivity.class);
           
       @Test
       public void testDPadNavigation() {
           // Test D-pad navigation between elements
           onView(withId(R.id.channel_grid))
               .perform(pressKey(KeyEvent.KEYCODE_DPAD_DOWN))
               .perform(pressKey(KeyEvent.KEYCODE_DPAD_SELECT));
           
           // Verify expected outcome
           onView(withId(R.id.player_view)).check(matches(isDisplayed()));
       }
   }
   ```

2. **Use Firebase Test Lab for TV Devices**
   - Upload your APK to Firebase Test Lab
   - Select Android TV devices in the test matrix
   - Run both robo tests and instrumentation tests

## Common Issues and Solutions

1. **D-pad Navigation Issues**
   - Make sure all focusable elements have proper `nextFocus` attributes
   - Use `android:focusable="true"` for all interactive elements

2. **Performance on Low-end Devices**
   - Optimize image loading and caching
   - Reduce layout complexity for TV interfaces
   - Implement efficient list recycling with RecyclerView

3. **Playback Problems**
   - Test with multiple video codecs and container formats
   - Implement proper error handling for stream issues
   - Consider using ExoPlayer for better format support

## Resources

- [Android TV Developer Guide](https://developer.android.com/training/tv)
- [Leanback Library Documentation](https://developer.android.com/training/tv/start/libraries.html)
- [UI Patterns for TV](https://designguidelines.withgoogle.com/android-tv/android-tv/overview.html)

## Testing Checklist

- [ ] App installs successfully on Android TV
- [ ] All UI elements are navigable via D-pad
- [ ] Media playback works for all supported formats
- [ ] App responds appropriately to playback errors
- [ ] Memory usage remains stable during extended use
- [ ] App handles network connectivity changes gracefully
- [ ] UI is readable from 10 feet away (TV viewing distance)
- [ ] App meets all Android TV quality guidelines
