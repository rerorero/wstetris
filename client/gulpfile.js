var gulp = require('gulp');
var browserify = require('browserify');
var babelify = require('babelify');
var source = require('vinyl-source-stream');
var webserver = require('gulp-webserver');

gulp.task('browserify', function() {
  browserify('./js/app.js', { debug: true })
    .transform(babelify)
    .bundle()
    .on("error", function (err) { console.log("Error : " + err.message); })
    .pipe(source('app.js'))
    .pipe(gulp.dest('./dist'))
});

gulp.task('html', function() {
  gulp.src('./html/*.html')
    .pipe(gulp.dest('./dist'))
});

gulp.task('watch', function() {
  gulp.watch('./js/*.js', ['browserify'])
  gulp.watch('./html/*.html', ['html'])
});

gulp.task('webserver', function() {
  gulp.src('./dist')
    .pipe(webserver({
      host: '192.168.0.4',
      livereload: true
    })
  );
});

gulp.task('default', ['browserify', 'html', 'watch', 'webserver']);
