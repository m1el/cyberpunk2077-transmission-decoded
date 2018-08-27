using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.IO;

namespace ConsoleApplication69
{
    class Program
    {
        static void Main(string[] args)
        {
            List<string> all = File.ReadAllLines("cyberpunk2077-raw.b64").ToList();

            var files = new List<List<string>>();
            var file = new List<string>();
            foreach (var line in all) {
                if (line.StartsWith("new file!"))
                {
                    files.Add(file);
                    file = new List<string>();
                }
                else
                {
                    if (string.IsNullOrWhiteSpace(line) || line.Length != 72 || line.Contains(' '))
                        continue;
                    file.Add(line);
                }
            }

            files.RemoveAt(0);
            var goodfiles = new List<List<string>>();
            goodfiles.Add(files[0]);
            for (int i = 1; i<files.Count; i++)
            {
                if (string.Join("", files[i].GetRange(0,3)) != string.Join("", files[i - 1].GetRange(0,3)))
                {
                    goodfiles.Add(files[i]);
                }
            }

            files = goodfiles;

            var lf = new LinesFreq();

            foreach (var f in files)
            {
                foreach (var l in f)
                {
                    lf.AddLine(l);
                }
            }

            lf.DumpData("shit_data.txt");

            var res = new List<string>();
            var res2 = new List<string>();
            res.AddRange(files[0].GetRange(0, 20));
            bool skipped = false;
            for (int i = 1; i < files.Count; i++)
            {
                var iol = files[i].IndexOf(res.Last());
                if (iol < 0)
                {
                    res2.Add(string.Format("Skipped file {0}, not found {1}\n", i, res.Last()));
                    continue;
                }
                iol++;
                while(iol < files[i].Count && lf.IsAuthentic(files[i][iol])) {
                    if (res.Last() != files[i][iol])
                    {
                        res.Add(files[i][iol]);
                    }
                    iol++;
                }
            }

            // some hardcoded lines at the end of the file
            res.Add("RE76XDp377rOkrb5m5zcO/C2DSZWIqJ9Gmw5Kbl+gvnKuObqzyJOsVPhGQDPJovtkB1BjfYN");
            res.Add("kv8/70Ku4e70cWYAAAAASUVORK5CYII=");

            File.WriteAllLines("cyberpunk2077-decoded.png.b64", res);
            File.WriteAllLines("whateverout2-difficult.txt", res2);
        }
    }

    public class LinesFreq
    {
        private Dictionary<string, int> _linesFreq;

        public LinesFreq()
        {
            _linesFreq = new Dictionary<string, int>();
            // this line is bugged for some reason
            _linesFreq.Add("NKFz87zGEacY1R4DCQ81iWMjABK+slCD93F6pf3CLM+Rh1wjDk3Be8L0T5KTsEveQaqWNO6j", 1);
        }

        public void AddLine(string line)
        {
            if (_linesFreq.ContainsKey(line))
            {
                _linesFreq[line]++;
            }
            else
            {
                _linesFreq.Add(line, 1);
            }
        }

        public bool IsAuthentic(string line)
        {
            return _linesFreq.ContainsKey(line) && _linesFreq[line] > 1;
        }

        public void DumpData(string filename)
        {
            var res = new List<string>();
            foreach (var k in _linesFreq.Keys)
            {
                res.Add(string.Format("{0}: {1}", _linesFreq[k], k));
            }
            File.WriteAllLines(filename, res);
        }
    }
}
