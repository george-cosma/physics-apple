all: make_shader make_frames make_fields run_simulation

make_shader: ./src/shaders/static-field.cu
	nvcc ./src/shaders/static-field.cu -ptx -o ./src/shaders/static-field.ptx

make_frames: ./hand_sample.mp4
	cd frames && ffmpeg -i ../hand_sample.mp4 image-%04d.png

make_fields:
	cargo run --profile release -- generate ./frames/ --gpu

run_simulation:
	cargo run --profile release -- simulate-sequence ./frames/

save: make_shader make_frames make_fields
	cargo run --profile release -- simulate-sequence ./frames/ --save-to-file
	ffmpeg -f image2 -r 25 -i 'render/render.%03d.png' -vcodec libx264 -crf 22 output.mp4

clean:
	rm -rf ./frames/*.png
	rm -rf ./frames/*.png.field
	rm -f ./src/shaders/static-field.ptx
	rm -f ./output.mp4
