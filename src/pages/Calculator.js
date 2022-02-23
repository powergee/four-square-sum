import { useState, forwardRef } from 'react';
import { TextField, FormGroup, FormControlLabel, Switch, Grid, Typography } from "@mui/material"
import { findSolution } from '../algorithms/js/FourSquare';
import './Calculator.css';
import 'katex/dist/katex.min.css';
import TeX from '@matejmazur/react-katex';
import NumberFormat from 'react-number-format';

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
  const [number, setNumber] = useState("0");
  const [result, setResult] = useState(["0", "0", "0", "0"]);
  const [findOptimal, setFindOptimal] = useState(false);

  function updateResult(nStr) {
    if (!nStr.includes('-')) {
      setResult(findSolution(nStr, findOptimal))
    }
  }

  function handleNumberChange(e) {
    setNumber(e.target.value);
    updateResult(e.target.value);
  }

  function handleOptimalChange(e) {
    setFindOptimal(e.target.checked);
    updateResult(number);
  }

  return (
    <div className='calc-root'>
      <FormGroup>
        <Grid container>
          <Grid item xs>
            <FormControlLabel label="Find a solution with the fewest number" control={
              <Switch checked={findOptimal} onChange={handleOptimalChange}></Switch>}>
            </FormControlLabel>
          </Grid>

          <Grid item xs="auto">
            <Typography variant='caption'>Current time complexity - </Typography>
            <TeX math={findOptimal ? "\\mathcal{O}(\\sqrt[4]{N})" : "\\mathcal{O}(\\log{}(N)^2)"}></TeX>
          </Grid>
        </Grid>
        

        <TextField
          value={number}
          onChange={handleNumberChange}
          variant="outlined"
          InputProps={{
            inputComponent: NumberFormatCustom,
          }}
          helperText={findOptimal ? "Due to the polynomial time complexity, the browser may stop with a large number." : undefined}>
        </TextField>

      </FormGroup>

      <div className='calc-results'>
        <TeX math={`${number}=${result.map(x => x+"^2").join("+")}`}></TeX>
      </div>
    </div>
  );
}

export default Calculator;