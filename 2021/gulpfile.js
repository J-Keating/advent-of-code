// === CONFIGURABLE VARIABLES

var bpfoldername = "creatortest";
var uwpBuild = false;

// === END CONFIGURABLE VARIABLES

const gulp = require("gulp");
const ts = require("gulp-typescript");
const del = require("del");
const os = require("os");
const GulpClient = require("gulp");

let mcdir =
  os.homedir() +
  (uwpBuild
    ? "/AppData/Local/Packages/Microsoft.MinecraftUWP_8wekyb3d8bbwe/LocalState/games/com.mojang/"
    : "/AppData/Roaming/MinecraftPE/games/com.mojang/");

function clean_build(cb) {
  del(["build/behavior_packs/", "build/resource_packs/"]).then(
    (value) => {
      cb(); // Success!
    },
    (reason) => {
      cb(); // Error!
    }
  );
}

function copy_behavior_packs() {
  return gulp.src(["behavior_packs/**/*"]).pipe(gulp.dest("build/behavior_packs"));
}

function copy_resource_packs() {
  return gulp.src(["resource_packs/**/*"]).pipe(gulp.dest("build/resource_packs"));
}

const copy_content = gulp.series(copy_behavior_packs, copy_resource_packs);

function compile_scripts() {
  return gulp
    .src("scripts/**/*.ts")
    .pipe(
      ts({
        module: "es2020",
        moduleResolution: "node",
        strict: true,
        noImplicitAny: true,
      })
    )
    .pipe(gulp.dest("build/behavior_packs/" + bpfoldername + "/scripts"));
}

const build = gulp.series(clean_build, copy_content, compile_scripts);

function clean_localmc(cb) {
  del([mcdir + "development_behavior_packs/" + bpfoldername, mcdir + "development_resource_packs/" + bpfoldername], {
    force: true,
  }).then(
    (value) => {
      cb(); // Success!
    },
    (reason) => {
      cb(); // Error!
    }
  );
}

function deploy_localmc_behavior_packs() {
  return gulp
    .src(["build/behavior_packs/" + bpfoldername + "/**/*"])
    .pipe(gulp.dest(mcdir + "development_behavior_packs/" + bpfoldername));
}

function deploy_localmc_resource_packs() {
  return gulp
    .src(["build/resource_packs/" + bpfoldername + "/**/*"])
    .pipe(gulp.dest(mcdir + "development_resource_packs/" + bpfoldername));
}

const deploy_localmc = gulp.series(
  clean_localmc,
  function (cb) {
    console.log("\007"); // annunciate a beep!
    cb();
  },
  deploy_localmc_behavior_packs,
  deploy_localmc_resource_packs
);

function watch() {
  return gulp.watch(
    ["scripts/**/*.ts", "behavior_packs/**/*", "resource_packs/**/*"],
    gulp.series(build, deploy_localmc)
  );
}

exports.clean_build = clean_build;
exports.copy_behavior_packs = copy_behavior_packs;
exports.copy_resource_packs = copy_resource_packs;
exports.compile_scripts = compile_scripts;
exports.copy_content = copy_content;
exports.build = build;
exports.clean_localmc = clean_localmc;
exports.deploy_localmc = deploy_localmc;
exports.default = gulp.series(build, deploy_localmc);
exports.clean = gulp.series(clean_build, clean_localmc);
exports.watch = watch;
