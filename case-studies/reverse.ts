const rev_sort = (s: int[], log = false): [int[], int] => {
  let revs = 0;
  let res = s;
  if (log) console.log(res.join());
  for (let iter = 0; iter < res.length; iter++) {
    for (let idx = iter; idx < res.length; idx++) {
      if (res[idx] < res[iter]) {
        res = [
          ...res.slice(0, iter),
          ...res.slice(iter, idx + 1).reverse(),
          ...res.slice(idx + 1),
        ];
        if (log) {
          //   const q =
          //     " ".repeat(iter * 2) +
          //     "+".repeat((idx + 1 - iter) * 2 - 1) +
          //     " ".repeat((res.length - (idx + 1)) * 2);
          //   console.log(q.slice(0, res.length * 2));
          //   console.log(res.join());

          console.log(
            res
              .map((n, i) => {
                const classes = ["box"];
                if (i <= iter) classes.push("sorted");
                return `\\node[${classes.join(
                  ","
                )}] at (${revs},${-i}) {${n}};`;
              })
              .join("\n")
          );
          console.log(`\\reveresed{${revs}}{${iter}}{${idx}};`);
        }
        revs += 1;
      }
    }
  }
  return [res, revs];
};

const randomList = (length: number) =>
  Array.from({ length }).map((_) => (1 + Math.random() * 10) | 0);

// console.log(rev_sort([4, 9, 2, 3, 7, 8, 4, 1]));

// let best_steps = 0;
// let best_list = [];

// for (let i = 0; i < 10000000; i++) {
//   const s = randomList(6);
//   const [_, steps] = rev_sort(s);
//   if (steps > best_steps) {
//     best_steps = steps;
//     best_list = s;
//   }
// }

const good_list = [8, 6, 5, 7, 9, 4];

console.log(rev_sort(good_list, true));
