#!/usr/bin/env python3

import openai
import sys
from pathlib import Path


def eprint(*args, **kwargs):
    import sys
    print(*args, file=sys.stderr, **kwargs)


#
# Setup
#
if len(sys.argv) != 2:
    eprint(f"""\
USAGE:
    {sys.argv[0]} FILE

DESCRIPTION:
    Generate spoken audio from text using OpenAI Text-to-Speech API.
    The audio file is output to the current directory as .mp3.
""")
    sys.exit(1)

txt_path = Path(sys.argv[1])

try:
    with open(txt_path, 'r') as file:
        text = file.read()
except OSError as e:
    eprint(f'open: {e}')
    sys.exit(1)

#
# Text-to-Speech
#
client = openai.OpenAI()

text_summary = repr(text[:16] + ' ... ' + text[-16:]
                    ) if len(text) > 32 else repr(text)
eprint(f'Generating spoken audio from {txt_path}: ' + text_summary)

with client.audio.speech.with_streaming_response.create(
    input=text,
    model="tts-1",
    voice="alloy"
) as response:
    mp3_path = txt_path.with_suffix('.mp3')
    response.stream_to_file(mp3_path.name)
    eprint(f'Generated spoken audio: {mp3_path.name}')