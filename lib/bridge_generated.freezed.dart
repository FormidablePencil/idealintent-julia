// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target

part of 'bridge_generated.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
    'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#custom-getters-and-methods');

/// @nodoc
mixin _$CompositionCategory {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(CarouselType field0) carousel,
    required TResult Function(BannerType field0) banner,
    required TResult Function(ParagraphType field0) paragraph,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(CarouselType field0)? carousel,
    TResult Function(BannerType field0)? banner,
    TResult Function(ParagraphType field0)? paragraph,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(CarouselType field0)? carousel,
    TResult Function(BannerType field0)? banner,
    TResult Function(ParagraphType field0)? paragraph,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Carousel value) carousel,
    required TResult Function(Banner value) banner,
    required TResult Function(Paragraph value) paragraph,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Carousel value)? carousel,
    TResult Function(Banner value)? banner,
    TResult Function(Paragraph value)? paragraph,
  }) =>
      throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Carousel value)? carousel,
    TResult Function(Banner value)? banner,
    TResult Function(Paragraph value)? paragraph,
    required TResult orElse(),
  }) =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $CompositionCategoryCopyWith<$Res> {
  factory $CompositionCategoryCopyWith(
          CompositionCategory value, $Res Function(CompositionCategory) then) =
      _$CompositionCategoryCopyWithImpl<$Res>;
}

/// @nodoc
class _$CompositionCategoryCopyWithImpl<$Res>
    implements $CompositionCategoryCopyWith<$Res> {
  _$CompositionCategoryCopyWithImpl(this._value, this._then);

  final CompositionCategory _value;
  // ignore: unused_field
  final $Res Function(CompositionCategory) _then;
}

/// @nodoc
abstract class _$$CarouselCopyWith<$Res> {
  factory _$$CarouselCopyWith(
          _$Carousel value, $Res Function(_$Carousel) then) =
      __$$CarouselCopyWithImpl<$Res>;
  $Res call({CarouselType field0});
}

/// @nodoc
class __$$CarouselCopyWithImpl<$Res>
    extends _$CompositionCategoryCopyWithImpl<$Res>
    implements _$$CarouselCopyWith<$Res> {
  __$$CarouselCopyWithImpl(_$Carousel _value, $Res Function(_$Carousel) _then)
      : super(_value, (v) => _then(v as _$Carousel));

  @override
  _$Carousel get _value => super._value as _$Carousel;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Carousel(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as CarouselType,
    ));
  }
}

/// @nodoc

class _$Carousel implements Carousel {
  const _$Carousel(this.field0);

  @override
  final CarouselType field0;

  @override
  String toString() {
    return 'CompositionCategory.carousel(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Carousel &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$CarouselCopyWith<_$Carousel> get copyWith =>
      __$$CarouselCopyWithImpl<_$Carousel>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(CarouselType field0) carousel,
    required TResult Function(BannerType field0) banner,
    required TResult Function(ParagraphType field0) paragraph,
  }) {
    return carousel(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(CarouselType field0)? carousel,
    TResult Function(BannerType field0)? banner,
    TResult Function(ParagraphType field0)? paragraph,
  }) {
    return carousel?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(CarouselType field0)? carousel,
    TResult Function(BannerType field0)? banner,
    TResult Function(ParagraphType field0)? paragraph,
    required TResult orElse(),
  }) {
    if (carousel != null) {
      return carousel(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Carousel value) carousel,
    required TResult Function(Banner value) banner,
    required TResult Function(Paragraph value) paragraph,
  }) {
    return carousel(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Carousel value)? carousel,
    TResult Function(Banner value)? banner,
    TResult Function(Paragraph value)? paragraph,
  }) {
    return carousel?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Carousel value)? carousel,
    TResult Function(Banner value)? banner,
    TResult Function(Paragraph value)? paragraph,
    required TResult orElse(),
  }) {
    if (carousel != null) {
      return carousel(this);
    }
    return orElse();
  }
}

abstract class Carousel implements CompositionCategory {
  const factory Carousel(final CarouselType field0) = _$Carousel;

  CarouselType get field0 => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  _$$CarouselCopyWith<_$Carousel> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$BannerCopyWith<$Res> {
  factory _$$BannerCopyWith(_$Banner value, $Res Function(_$Banner) then) =
      __$$BannerCopyWithImpl<$Res>;
  $Res call({BannerType field0});
}

/// @nodoc
class __$$BannerCopyWithImpl<$Res>
    extends _$CompositionCategoryCopyWithImpl<$Res>
    implements _$$BannerCopyWith<$Res> {
  __$$BannerCopyWithImpl(_$Banner _value, $Res Function(_$Banner) _then)
      : super(_value, (v) => _then(v as _$Banner));

  @override
  _$Banner get _value => super._value as _$Banner;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Banner(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as BannerType,
    ));
  }
}

/// @nodoc

class _$Banner implements Banner {
  const _$Banner(this.field0);

  @override
  final BannerType field0;

  @override
  String toString() {
    return 'CompositionCategory.banner(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Banner &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$BannerCopyWith<_$Banner> get copyWith =>
      __$$BannerCopyWithImpl<_$Banner>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(CarouselType field0) carousel,
    required TResult Function(BannerType field0) banner,
    required TResult Function(ParagraphType field0) paragraph,
  }) {
    return banner(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(CarouselType field0)? carousel,
    TResult Function(BannerType field0)? banner,
    TResult Function(ParagraphType field0)? paragraph,
  }) {
    return banner?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(CarouselType field0)? carousel,
    TResult Function(BannerType field0)? banner,
    TResult Function(ParagraphType field0)? paragraph,
    required TResult orElse(),
  }) {
    if (banner != null) {
      return banner(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Carousel value) carousel,
    required TResult Function(Banner value) banner,
    required TResult Function(Paragraph value) paragraph,
  }) {
    return banner(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Carousel value)? carousel,
    TResult Function(Banner value)? banner,
    TResult Function(Paragraph value)? paragraph,
  }) {
    return banner?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Carousel value)? carousel,
    TResult Function(Banner value)? banner,
    TResult Function(Paragraph value)? paragraph,
    required TResult orElse(),
  }) {
    if (banner != null) {
      return banner(this);
    }
    return orElse();
  }
}

abstract class Banner implements CompositionCategory {
  const factory Banner(final BannerType field0) = _$Banner;

  BannerType get field0 => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  _$$BannerCopyWith<_$Banner> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$ParagraphCopyWith<$Res> {
  factory _$$ParagraphCopyWith(
          _$Paragraph value, $Res Function(_$Paragraph) then) =
      __$$ParagraphCopyWithImpl<$Res>;
  $Res call({ParagraphType field0});
}

/// @nodoc
class __$$ParagraphCopyWithImpl<$Res>
    extends _$CompositionCategoryCopyWithImpl<$Res>
    implements _$$ParagraphCopyWith<$Res> {
  __$$ParagraphCopyWithImpl(
      _$Paragraph _value, $Res Function(_$Paragraph) _then)
      : super(_value, (v) => _then(v as _$Paragraph));

  @override
  _$Paragraph get _value => super._value as _$Paragraph;

  @override
  $Res call({
    Object? field0 = freezed,
  }) {
    return _then(_$Paragraph(
      field0 == freezed
          ? _value.field0
          : field0 // ignore: cast_nullable_to_non_nullable
              as ParagraphType,
    ));
  }
}

/// @nodoc

class _$Paragraph implements Paragraph {
  const _$Paragraph(this.field0);

  @override
  final ParagraphType field0;

  @override
  String toString() {
    return 'CompositionCategory.paragraph(field0: $field0)';
  }

  @override
  bool operator ==(dynamic other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$Paragraph &&
            const DeepCollectionEquality().equals(other.field0, field0));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(field0));

  @JsonKey(ignore: true)
  @override
  _$$ParagraphCopyWith<_$Paragraph> get copyWith =>
      __$$ParagraphCopyWithImpl<_$Paragraph>(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(CarouselType field0) carousel,
    required TResult Function(BannerType field0) banner,
    required TResult Function(ParagraphType field0) paragraph,
  }) {
    return paragraph(field0);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult Function(CarouselType field0)? carousel,
    TResult Function(BannerType field0)? banner,
    TResult Function(ParagraphType field0)? paragraph,
  }) {
    return paragraph?.call(field0);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(CarouselType field0)? carousel,
    TResult Function(BannerType field0)? banner,
    TResult Function(ParagraphType field0)? paragraph,
    required TResult orElse(),
  }) {
    if (paragraph != null) {
      return paragraph(field0);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(Carousel value) carousel,
    required TResult Function(Banner value) banner,
    required TResult Function(Paragraph value) paragraph,
  }) {
    return paragraph(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult Function(Carousel value)? carousel,
    TResult Function(Banner value)? banner,
    TResult Function(Paragraph value)? paragraph,
  }) {
    return paragraph?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(Carousel value)? carousel,
    TResult Function(Banner value)? banner,
    TResult Function(Paragraph value)? paragraph,
    required TResult orElse(),
  }) {
    if (paragraph != null) {
      return paragraph(this);
    }
    return orElse();
  }
}

abstract class Paragraph implements CompositionCategory {
  const factory Paragraph(final ParagraphType field0) = _$Paragraph;

  ParagraphType get field0 => throw _privateConstructorUsedError;
  @JsonKey(ignore: true)
  _$$ParagraphCopyWith<_$Paragraph> get copyWith =>
      throw _privateConstructorUsedError;
}
