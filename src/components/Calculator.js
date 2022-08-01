import { useState, forwardRef, useEffect } from 'react';
import { TextField, FormGroup, FormControlLabel, Switch, Grid, Typography, InputAdornment, Paper } from "@mui/material";
import { findSolution as findSolutionJs } from '../algorithms/js/FourSquare';
import init, { findSolution as findSolutionRust } from "four-square-sum-wasm";

import './Calculator.css';
import 'katex/dist/katex.min.css';
import TeX from '@matejmazur/react-katex';

import NumberFormat from 'react-number-format';
import ReportGmailerrorredIcon from '@mui/icons-material/ReportGmailerrorred';

const NumberFormatCustom = forwardRef(function NumberFormatCustom(props, ref) {
  const { onChange, ...other } = props;

  return (
    <NumberFormat
      {...other}
      getInputRef={ref}
      onValueChange={(values) => {
        onChange({
          target: {
            name: props.name,
            value: values.value,
          },
        });
      }}
      thousandSeparator
      isNumericString
      prefix='N = '
    />
  );
});

function Calculator() {
  const [loadedWasm, setLoadedWasm] = useState(false);
  const [useWasm, setUseWasm] = useState(true);
  const [number, setNumber] = useState("0");
  const [result, setResult] = useState(["0", "0", "0", "0"]);
  const [findOptimal, setFindOptimal] = useState(false);

  useEffect(() => {
    init().then(() => {
      setLoadedWasm(true);
    });
  }, []);

  useEffect(() => {
    const callJs = (nStr, findOptimal) => findSolutionJs(nStr, findOptimal);
    const callRust = (nStr, findOptimal) => findSolutionRust(nStr, findOptimal).split(" ");

    let call = callJs;
    if (loadedWasm && useWasm) {
      call = callRust;
    }
    if (number && !number.includes("-") && number !== "0") {
      setResult(call(number, findOptimal));
    }
  }, [number, findOptimal, useWasm, loadedWasm]);

  function handleNumberChange(e) {
    setNumber(e.target.value);
  }

  function handleOptimalChange(e) {
    setFindOptimal(e.target.checked);
  }

  function handleUseWasmChange(e) {
    setUseWasm(e.target.checked);
  }

  function getHelperText() {
    if (findOptimal) {
      return "Due to the heavy polynomial time complexity, the browser may stop with a large number.";
    }
    if (!loadedWasm) {
      return "Loading Wasm(Rust)...";
    }
    return undefined;
  }

  return (
    <div className='calc-root'>
      <Paper variant='outlined' className='calc-paper'>
        <FormGroup>
          <Grid container>
            <Grid item xs>
              <FormControlLabel label="Use WebAssembly" control={
                <Switch checked={useWasm} onChange={handleUseWasmChange}></Switch>}>
              </FormControlLabel>
              <FormControlLabel label="Find a solution with the fewest number" control={
                <Switch checked={findOptimal} onChange={handleOptimalChange}></Switch>}>
              </FormControlLabel>
            </Grid>

            <Grid item xs="auto">
              <Typography variant='caption'>Current time complexity - </Typography>
              <TeX math={findOptimal ? "\\mathcal{O}(\\sqrt[4]{N})" : "\\text{Randomized polynomial-time}"}></TeX>
            </Grid>
          </Grid>
          
          <TextField
            value={number}
            onChange={handleNumberChange}
            variant="outlined"
            InputProps={{
              startAdornment: (findOptimal &&
                <InputAdornment position="start">
                  <ReportGmailerrorredIcon />
                </InputAdornment>
              ),
              inputComponent: NumberFormatCustom,
            }}
            helperText={getHelperText()}>
          </TextField>

        </FormGroup>

        <div className='calc-results'>
          <TeX math={`${number}=${result.map(x => x+"^2").join("+")}`}></TeX>
        </div>
      </Paper>
    </div>
  );
}

export default Calculator;