use strict;
use warnings;

sub PrintList {
    my (@list) = @_;

    foreach (@list) { print "$_\n" }
}
