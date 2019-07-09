 {$LONGSTRINGS ON}

 program palindrome;

type
   DPEntry = record
                start: integer;
                length: integer
             end;
   DPT = array of array of DPEntry;

function max_dpentry(a: DPEntry; b: DPEntry): DPEntry;
begin
   if a.length > b.length then
      max_dpentry := a
   else
      max_dpentry := b
end;

procedure fill_dp(var dp: DPT; s: string; i: integer; cols: integer);
var
   j: integer;
   lo, hi: integer;
   new_best: boolean;
begin
   for j := 0 to (cols - i) do begin
      lo := j+1;
      hi := j+i;

      {
      There's a new, better, palindrome if:
      1. s[lo] = s[hi]
      2. The previous best palindrome was 2 characters shorter
      3. The previous best palindrome started at j+1
      }
      new_best :=
        (s[lo] = s[hi])
        and (dp[i-2, j+1].length = i - 2)
        and (dp[i-2, j+1].start = j + 1);

      if new_best then begin
         dp[i, j].start := j;
         dp[i, j].length := 2 + dp[i-2, j+1].length
      end
      else if j = 0 then
         dp[i, j] := dp[i-1, j]
      else
         dp[i, j] := max_dpentry(dp[i, j-1], dp[i-1, j])
   end;

   for j := (cols - i + 1) to cols - 1 do
      dp[i, j] := max_dpentry(dp[i, j-1], dp[i-1, j])
end;

procedure write_palindrome(s: string; dpe: DPEntry);
var
   i: integer;
begin
   for i := (dpe.start + 1) to (dpe.start + dpe.length) do
      write(s[i]);
   writeln()
end;

procedure write_dp(var dp: DPT; rows: integer; cols: integer);
var
   i, j: integer;
begin
   for i := 0 to rows - 1 do begin
      for j := 0 to cols - 1 do
         write(dp[i, j].start, ',', dp[i,j].length, '    ');
      writeln()
   end
end;

var
   dp: DPT;
   rows: integer;
   cols: integer;
   i, j: integer;
   input: string;
   best: DPEntry;
begin
   read(input);
   cols := length(input);
   rows := cols + 1;
   setlength(dp, rows, cols);

   { Initialize first two rows. }
   for i := 0 to 1 do
      for j := 0 to cols - 1 do begin
         dp[i, j].start := j;
         dp[i, j].length := i
      end;

   for i := 2 to rows - 1 do
      fill_dp(dp, input, i, cols);

   best := dp[rows-1, cols-1];
   write_palindrome(input, best)
end.
