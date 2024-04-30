# SPDX-License-Identifier: Apache-2.0
# Author: Santiago Zarate <github@zarate.co>
use strict;
use warnings;
use feature 'say';
use HTTP::Tiny;


sub showtime {
  say time;
  open my $fh, '+>>', 'hello-from-rust' or die $!;
  print $fh "Hello from rust $fh\n";
  close $fh;
}

sub get_quick_headers {
  my $response = HTTP::Tiny->new->get('https://foursixnine.io');

  die "Failed!\n" unless $response->{success};
  say 'getting headers...';

  while (my ($k, $v) = each %{$response->{headers}}) {
      for (ref $v eq 'ARRAY' ? @$v : $v) {
          print "$k: $_\n";
      }
  }

}
1;