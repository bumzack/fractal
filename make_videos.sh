#!/bin/zsh

ffmpeg -framerate 60 -pattern_type glob -i '*.png' -c:v libx265  -crf 50   -b 40M  -pix_fmt yuv420p tendril_40b_original_60fps_crf_50.mp4
ffmpeg -framerate 60  -s 1920x1080  -pattern_type glob -i '*.png' -c:v libx265  -crf 50   -b 40M  -pix_fmt yuv420p tendril_40b_1920x1080_60fps_crf_50.mp4

ffmpeg -framerate 30 -pattern_type glob -i '*.png' -c:v libx265  -crf 50   -b 40M  -pix_fmt yuv420p tendril_40b_original_30fps_crf_50.mp4
ffmpeg -framerate 30  -s 1920x1080  -pattern_type glob -i '*.png' -c:v libx265  -crf 50   -b 40M  -pix_fmt yuv420p tendril_40b_1920x1080_30fps_crf_50.mp4

ffmpeg -framerate 60 -pattern_type glob -i '*.png' -c:v libx265  -crf 25   -b 40M  -pix_fmt yuv420p tendril_40b_original_60fps_crf_25.mp4
ffmpeg -framerate 60  -s 1920x1080  -pattern_type glob -i '*.png' -c:v libx265  -crf 25   -b 40M  -pix_fmt yuv420p tendril_40b_1920x1080_60fps_crf_25.mp4

ffmpeg -framerate 30 -pattern_type glob -i '*.png' -c:v libx265  -crf 25   -b 40M  -pix_fmt yuv420p tendril_40b_original_30fps_crf_25.mp4
ffmpeg -framerate 30  -s 1920x1080  -pattern_type glob -i '*.png' -c:v libx265  -crf 25   -b 40M  -pix_fmt yuv420p tendril_40b_1920x1080_30fps_crf_25.mp4

ffmpeg -framerate 60 -pattern_type glob -i '*.png' -c:v libx265  -crf 50   -b 20M  -pix_fmt yuv420p tendril_20b_original_60fps_crf_50.mp4
ffmpeg -framerate 60  -s 1920x1080  -pattern_type glob -i '*.png' -c:v libx265  -crf  50   -b 20M  -pix_fmt yuv420p tendril_20b_1920x1080_60fps_crf_50.mp4


