import { Component, createElement as h, createRef } from 'react';


export default ({
  layerCount,
  name,
  runner
}) => class Algorithm extends Component {
  static displayName = `Algorithm:${name}`;

  canvasRef = createRef();
  run = [];

  componentDidMount() {
    const canvas = this.canvasRef.current;
    const dpr = window.devicePixelRatio || 1;
    const bounds = canvas.getBoundingClientRect();

    canvas.width = bounds.width * dpr;
    canvas.height = bounds.height * dpr;

    this.ctx = canvas.getContext('2d');
    this.ctx.scale(dpr, dpr);
    this.run = runner({
      dimensions: this.props.dimensions,
      cellSize: this.props.cellSize
    });
    this.props.setStepCount(this.run.length);
  }

  componentDidUpdate(prevProps) {
    if (this.props.step === prevProps.step) return;

    const prevStep = prevProps.step;
    const nextStep = this.props.step;
    const { run, ctx } = this;

    if (0 > nextStep || nextStep > run.length) {
      throw new Error(`Next step ${nextStep} is out of bounds`);
    }

    if (nextStep >= prevStep) {
      for (let i = prevStep; i < nextStep; i++) {
        ctx.fillStyle = run[i].after;
        ctx.fillRect(...run[i].bounds);
      }
    } else {
      for (let i = prevStep - 1; i >= nextStep; i--) {
        ctx.fillStyle = run[i].before;
        ctx.fillRect(...run[i].bounds);
      }
    }
  }

  render() {
    const [x, y] = this.props.dimensions;

    return h('canvas', {
      ref: this.canvasRef,
      style: {
        width: x * this.props.cellSize,
        height: y * this.props.cellSize
      }
    });
  }
}
