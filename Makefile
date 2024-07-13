nightly:
	rustup default nightly

stable:
	rustup default stable

before_build_android:
	rustup target add \
        aarch64-linux-android \
        armv7-linux-androideabi \
        x86_64-linux-android \
        i686-linux-android

build_android:
	export ANDROID_NDK_HOME=$HOME/Library/Android/sdk/ndk/25.0.8775105
	cargo ndk \
		-t armeabi-v7a \
		-t arm64-v8a \
		-t x86_64 \
		-t x86 \
		-o dist/android/jniLibs build --release

before_build_ios:
	rustup target add aarch64-apple-ios x86_64-apple-ios
	cargo install cargo-lipo

build_ios:
	export ANDROID_NDK_HOME=$HOME/Library/Android/sdk/ndk/25.0.8775105
	cargo lipo --release && cp target/universal/release/libfqrs.a dist/ios/Classes

bench:
	cargo bench --bench quircs