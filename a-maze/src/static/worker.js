importScripts("./pkg/a_maze.js");

delete WebAssembly.instantiateStreaming;
pkg("./pkg/a_maze_bg.wasm").then(
  wasm => {
    pkg.a_maze_init();
    const { Maze, Generator } = pkg;

    let maze;
    let generator;
    let step = 0;

    const postSuccess = id => postMessage({
      id,
      payload: { success: true }
    });

    const postRender = () => {
      const width = maze.width();
      const height = maze.height();
      const stepCount = maze.step_count();

      postMessage({
        id: 'render',
        payload: {
          buffer: new Uint8ClampedArray(
            wasm.memory.buffer,
            maze.image_data(),
            width * height * 4
          ),
          width,
          height,
          stepCount,
          step
        }
      });
    };

    onmessage = ({ data: { type, payload, id } }) => {
      switch (type) {
        case "SETUP": {
          const { cellSize, wallSize, maxWidth, maxHeight } = payload;
          maze = pkg.Maze.new(cellSize, wallSize, maxWidth, maxHeight);
          postSuccess(id);
          postRender();
          break;
        }

        case "SET_STEP": {
          maze.set_step(payload);
          step = payload;
          postSuccess(id);
          postRender();
          break;
        }

        case "SET_GENERATOR": {
          const newGenerator = {
            hilburt: Generator.Hilburt,
            random: Generator.Random,
            test: Generator.Test
          }[payload];

          if (newGenerator === undefined) {
            postMessage({
              id,
              error: `Unknown generator payload '${payload}'`
            });
          } else {
            generator = newGenerator;

            if (maze === undefined) {
              postSuccess(id);
            } else {
              maze.set_generator(generator);
              postSuccess(id);
              postRender();
            }
          }

          break;
        }

        default: {
          postMessage({
            id,
            error: `Unknown message type '${type}'`
          });
          break;
        }
      }
    };

    postSuccess("init");
  },
  error => {
    postMessage({
      id: "init",
      error
    });
  }
);
