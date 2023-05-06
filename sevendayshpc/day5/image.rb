#!/bin/ruby

# Copyright (c) https://github.com/kaityo256
# https://github.com/kaityo256/sevendayshpc/blob/f2a2cc595ccbd768894cf9d5d148c56e13be65b8/day5/image.rb

# NOTE:
# apt install ruby-dev
# gem install cairo
require "cairo"
require "pathname"

def convert(datfile)
  puts datfile
  buf = File.binread(datfile).unpack("d*")
  l = Math.sqrt(buf.size).to_i
  m = 4
  size = l * m

  surface = Cairo::ImageSurface.new(Cairo::FORMAT_RGB24, size, size)
  context = Cairo::Context.new(surface)
  context.set_source_rgb(1, 1, 1)
  context.rectangle(0, 0, size, size)
  context.fill

  l.times do |x|
    l.times do |y|
      u = buf[x + y * l]
      context.set_source_rgb(0, u, 0)
      context.rectangle(x * m, y * m, m, m)
      context.fill
    end
  end
  pngfile = Pathname(datfile).sub_ext(".png").to_s
  surface.write_to_png(pngfile)
end

`ls data/*.dat`.split(/\n/).each do |f|
  convert(f)
end
