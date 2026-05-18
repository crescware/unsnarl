import { A, B, C, D, E } from "components";

function Main() {
  const v = "v";
  const w = "w";
  const x = "x";
  const y = "y";
  const z = "z";

  return (
    <A>
      {v}
      <B>
        {w}
        <C>
          {x}
          <D>
            {y}
            <E>{z}</E>
          </D>
        </C>
      </B>
    </A>
  );
}
