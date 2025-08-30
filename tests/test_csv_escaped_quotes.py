import pytest
from veloxx import DataFrame

def test_csv_escaped_quotes(tmp_path):
    csv_content = 'name,desc\n"John","He said ""hello"" to everyone"\n"Jane","No quotes here"\n'
    csv_file = tmp_path / "test.csv"
    csv_file.write_text(csv_content)
    df = DataFrame.read_csv(str(csv_file))
    assert df["desc"].to_list()[0] == 'He said "hello" to everyone'
    assert df["desc"].to_list()[1] == 'No quotes here'
