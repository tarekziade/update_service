# inspired from https://github.com/yxd-hde/lambda-rust-demo/blob/master/main.py
from ctypes import cdll
import json
import os


CURDIR = os.path.dirname(__file__)
LIB_NAME = ('libupdate_service.dylib',
            'libupdate_service.so')
lib = None


for NAME in LIB_NAME:
    libpath = os.path.join(CURDIR, NAME)
    if not os.path.exists(libpath):
        continue

    lib = cdll.LoadLibrary(libpath)


def handler(event, context):
    ret = lib.handle(json.dumps(event),
                     json.dumps(context, default=encode_context))
    return ret


def encode_context(context):
    return {"function_name": context.function_name,
            "function_version": context.function_version,
            "invoked_function_arn": context.invoked_function_arn,
            "memory_limit_in_mb": context.memory_limit_in_mb,
            "aws_request_id": context.aws_request_id,
            "log_group_name": context.log_group_name,
            "log_stream_name": context.log_stream_name}


if __name__ == '__main__':
    event = {'s3_bucket': 'firefoxpoll',
             's3_filename': 'updates.json',
             'kinto_url': 'https://firefox.settings.services.mozilla.com/v1/buckets/monitor/collections/changes/records'
             }
    handler(event, {})



