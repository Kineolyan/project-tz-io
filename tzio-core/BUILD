#java_binary(
#    name = "tzio-spec",
#    srcs = glob(["src/main/java/**/*.java"]),
#)
java_library(
    name = "everything",
    srcs = glob([
        "spec/src/main/java/**/*.java",
        "tck/src/main/java/**/*.java",
        "java/src/main/java/**/*.java"
#        "jdk18/src/main/java/**/*.java"
    ]),
    resources = glob([
        "java/src/main/resources/**"
#        "jdk18/src/main/resources/**"
    ]),
    deps = ["//:all-external-targets"],
)